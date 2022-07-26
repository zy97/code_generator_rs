{% set dto = entity ~ "Dto" -%}
{% set camelName = entity|camel -%}
{% set store = camelName ~ "Store" -%}
{% set entities = entity|plural -%}

import { useAntdTable, useRequest } from "ahooks";
import { Button, Form, Input, message, Modal, Table } from "antd";
import { useState } from "react";
import AdvancedSearchForm from "";
import useStores from "";

function {{entity}}() {
  const { {{store}} } = useStores();
  const [modalTitle, setModalTitle] = useState("");
  const [visible, setVisible] = useState(false);
  const [form] = Form.useForm();
  const [modalForm] = Form.useForm();
  const { tableProps, search } = useAntdTable({{store}}.get{{entities}}, { defaultPageSize: 10, form, debounceWait: 500 });
  const { runAsync } = useRequest({{store}}.get{{entity}}ById, { manual: true });
  const delete{{entity}} = (record: {{dto}}) => {
    Modal.confirm({
      title: "删除标签", content: "确定删除吗？",
      onOk: async () => {
        const success = await {{store}}.delete{{entity}}(record.id);
        if (success) {
          message.success("删除成功");
          search.submit();
        } else {
          message.error("删除失败");
        }
      },
      okText: "确定", cancelText: "取消",
    });
  };
  const showModal = () => {
    setModalTitle("添加{{entity}}");
    setVisible(true);
  };
  const get{{entity}} = async (record: {{dto}}) => {
    try {
      const {{camelName}} = await runAsync(record.id);
      if ({{camelName}}) {
        setModalTitle("编辑{{entity}}");
        modalForm.setFieldsValue({{camelName}});
        console.log({{camelName}});
        setVisible(true);
      }
    } catch (error) { return; }
  };
  const addOrUpdate{{entity}} = async (data: {{dto}}) => {
    try {
      if (data.id) {
        const {{camelName}} = await {{store}}.update{{entity}}(data.id, data);
        if ({{camelName}}) {
          modalForm.resetFields();
          message.success("更新成功");
          setVisible(false);
          search.submit();
        }
      } else {
        const {{camelName}} = await {{store}}.add{{entity}}(data);
        if ({{camelName}}) {
          modalForm.resetFields();
          message.success("添加成功");
          setVisible(false);
          search.submit();
        }
      }
    } catch (error) { return; }
  };
  return (
    <div>
      {% if queries|length != 0 %}
      <AdvancedSearchForm form={form} {...search} extraActions={[{ content: "添加", action: showModal }]}>
        {% for query in queries %}
           <Form.Item name="{{query.0}}" label="{{query.0}}"><Input placeholder="请输入{{query.0}}" /></Form.Item>
        {%- endfor %}
      </AdvancedSearchForm>
      {% else %}
      {/* 此生成只是固定生成，如果想生成具体的，请在model里声明 export interface SearchXXXDto extends PageRequest*/}
       <AdvancedSearchForm form={form} {...search} extraActions={[{ content: "添加", action: showModal }]}>
           <Form.Item name="test" label="test"><Input placeholder="请输入test" /></Form.Item>
      </AdvancedSearchForm>
      {% endif %}
      <div className="mt-4">
        <Table<{{dto}}> rowKey="id"
          {...{
            ...tableProps,
            pagination: {
              ...tableProps.pagination,
              showTotal: (total) => {
                return <div>总共：{total} 项</div>;
              },
              showSizeChanger: true
          }
        }}>
        {% for property in properties -%}
          <Table.Column<{{dto}}> title="{{property.0}}" dataIndex="{{property.0}}" {% if property.1 == "boolean" %}render={(value) => <div>{value === true ? "是" : "否"}</div>}{% endif %}/>
        {% endfor%}
          <Table.Column<{{dto}}> title="操作"
            render={(recode) => {
              return (
                <div className="space-x-4">
                  <Button type="primary" onClick={() => get{{entity}}(recode)}>编辑</Button>
                  <Button type="primary" danger onClick={() => delete{{entity}}(recode)}>删除</Button>
                </div>
              );
            }} />
        </Table>
      </div>
      <Modal visible={visible} title={modalTitle} okText="确定" cancelText="取消"
        onCancel={() => {
          setVisible(false);
          modalForm.resetFields();
        }}
        onOk={() => {
          modalForm
            .validateFields()
            .then((values) => {
              addOrUpdate{{entity}}(values);
            })
            .catch(() => { message.error("添加失败"); });
        }}>
{%- raw %}
        <Form form={modalForm} name="form_in_modal" labelCol={{ span: 6 }} wrapperCol={{ span: 18 }}>
{%- endraw %}
          <Form.Item name="id" label="id" hidden><Input /></Form.Item>
        {%- for property in properties %}
          <Form.Item name="{{property.0}}" label="{{property.0}}" rules={[{ required: true, message: "请输入{{property.0}}" }]}><Input /></Form.Item>
        {%- endfor %}
        </Form>
      </Modal>
    </div>
  );
}
export default {{entity}};

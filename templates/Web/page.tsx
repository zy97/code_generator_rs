{% set dto = entity ~ "Dto" -%}
{% set snakeName = entity|snake -%}
{% set store = snakeName ~ "Store" -%}
{% set entities = entity|plural -%}

import { useAntdTable, useRequest } from "ahooks";
import { Button, Form, Input, message, Modal, Table } from "antd";
import { useState } from "react";
import AdvancedSearchForm from "../../../../components/AdvanceSearchForm";
import { {{entity}}Dto } from "../../../../data/models/{{entity}}";
import useStores from "../../../../hooks/useStore";

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
      const {{snakeName}} = await runAsync(record.id);
      if ({{snakeName}}) {
        setModalTitle("编辑{{entity}}");
        modalForm.setFieldsValue({{snakeName}});
        console.log({{snakeName}});
        setVisible(true);
      }
    } catch (error) { return; }
  };
  const addOrUpdate{{entity}} = async (data: {{dto}}) => {
    try {
      if (data.id) {
        const {{snakeName}} = await {{store}}.update{{entity}}(data.id, data);
        if ({{snakeName}}) {
          modalForm.resetFields();
          message.success("更新成功");
          setVisible(false);
          search.submit();
        }
      } else {
        const {{snakeName}} = await {{store}}.add{{entity}}(data);
        if ({{snakeName}}) {
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
      <AdvancedSearchForm form={form} {...search} extraActions={[{ content: "添加", action: showModal }]}>
        <Form.Item name="title" label="标题"><Input placeholder="请输入标题" /></Form.Item>
        <Form.Item name="linkUrl" label="链接地址"><Input placeholder="请输入链接地址" /></Form.Item>
      </AdvancedSearchForm>
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

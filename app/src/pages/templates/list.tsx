// import { IResourceComponentsProps } from "@refinedev/core";
// import { AntdListInferencer } from "@refinedev/inferencer/antd";

// export const TemplateList: React.FC<IResourceComponentsProps> = () => {
//   return <AntdListInferencer />;
// };

//按照例子来水，blog-post能展示外键关系，但这个模块不能，所以完全复制模板内容进行修改
import React from "react";
import { IResourceComponentsProps, BaseRecord, useMany } from "@refinedev/core";
import {
  useTable,
  List,
  EditButton,
  ShowButton,
  DeleteButton,
} from "@refinedev/antd";
import { Table, Space } from "antd";
import { SnippetsOutlined } from "@ant-design/icons";
import { redirect, useNavigate, useNavigation } from "react-router-dom";

export const TemplateList: React.FC<IResourceComponentsProps> = () => {
  const navigate = useNavigate();
  const { tableProps } = useTable({
    syncWithLocation: true,
  });

  const { data: projectData, isLoading: projectIsLoading } = useMany({
    resource: "projects",
    ids: tableProps?.dataSource?.map((item) => item?.project?.id) ?? [],
    queryOptions: {
      enabled: !!tableProps?.dataSource,
    },
  });
  const render = (id) => {
    let url = `/templates/render/${id}`;
    console.log("url", url);
    navigate(url);
  };
  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="Id" />
        <Table.Column dataIndex="name" title="Name" />
        {/* <Table.Column
          dataIndex="content"
          title="Content"
          ellipsis={{ showTitle: true }}
          width="200px"
        /> */}
        <Table.Column
          dataIndex={["project", "id"]}
          title="Project"
          render={(value) =>
            projectIsLoading ? (
              <>Loading...</>
            ) : (
              projectData?.data?.find((item) => item.id === value)?.title
            )
          }
        />
        <Table.Column
          title="Actions"
          dataIndex="actions"
          render={(_, record: BaseRecord) => (
            <Space>
              <EditButton hideText size="small" recordItemId={record.id} />
              <ShowButton hideText size="small" recordItemId={record.id} />
              <DeleteButton hideText size="small" recordItemId={record.id} />
              <EditButton
                hideText
                size="small"
                onClick={() => render(record.id)}
                recordItemId={record.id}
                icon={<SnippetsOutlined />}
              />
            </Space>
          )}
        />
      </Table>
    </List>
  );
};

import React, { useRef, useState } from "react";
import { IResourceComponentsProps } from "@refinedev/core";
import { Edit, useForm, useSelect } from "@refinedev/antd";
import { Col, Form, Input, Row, Select } from "antd";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from "react-syntax-highlighter/dist/esm/styles/hljs";
export const TemplateEdit: React.FC<IResourceComponentsProps> = () => {
  const { formProps, saveButtonProps, queryResult } = useForm();
  const codeString = "(num) => num + 1";
  const templatesData = queryResult?.data?.data;
  const textArea = useRef(null);
  const [code, setCode] = useState("");

  const { selectProps: projectSelectProps } = useSelect({
    resource: "projects",
    defaultValue: templatesData?.project_id,
  });
  const textAreaChange = (e) => {
    setCode(e.target.value);
  };
  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Row gutter={{ xs: 8, sm: 16, md: 24 }}>
        <Col span={12}>
          <Form {...formProps} layout="vertical">
            <Form.Item
              label="Id"
              hidden
              name={["id"]}
              rules={[
                {
                  required: true,
                },
              ]}
            >
              <Input readOnly disabled />
            </Form.Item>
            <Form.Item
              label="Name"
              name={["name"]}
              rules={[
                {
                  required: true,
                },
              ]}
            >
              <Input />
            </Form.Item>
            <Form.Item
              label="Project"
              name={"project_id"}
              rules={[
                {
                  required: true,
                },
              ]}
            >
              <Select {...projectSelectProps} />
            </Form.Item>
            <Form.Item
              label="Content"
              name={["content"]}
              rules={[
                {
                  required: true,
                },
              ]}
            >
              <Input.TextArea autoSize={true} onChange={textAreaChange} />
            </Form.Item>
          </Form>
        </Col>
        <Col span={12}>
          <SyntaxHighlighter language="csharp" style={docco} children={code} />
        </Col>
      </Row>
    </Edit>
  );
};

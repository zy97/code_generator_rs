import React, { useEffect, useState } from "react";
import { IResourceComponentsProps } from "@refinedev/core";
import { Create, useForm, useSelect } from "@refinedev/antd";
import { Col, Form, Input, Row, Select } from "antd";
import { invoke } from "@tauri-apps/api";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from "react-syntax-highlighter/dist/esm/styles/hljs";
import TagManager from "../../components/tag-manager";
const command = "get_expressions";
export const TemplateCreate: React.FC<IResourceComponentsProps> = () => {
  const { formProps, saveButtonProps, queryResult, onFinish } = useForm({});
  const [expressions, setExpressions] = useState<string[]>([]);
  const { selectProps: projectSelectProps } = useSelect({
    optionLabel: "name",
    resource: "projects",
  });
  useEffect(() => {
    console.log("projectSelectProps", projectSelectProps);
  }, [projectSelectProps]);

  const [code, setCode] = useState("");
  const onSubmit = (e) => {
    e.preventDefault();
    console.log("formProps", formProps.form);
  };
  const onFinish1 = (values) => {
    console.log("Received values of form: ", values);
    onFinish({ ...values, projectId: values.project.id });
  };
  const textAreaChange = async (e) => {
    const code = e.target.value;
    const expressions = await invoke(command, { template: code });
    setExpressions((expressions as string[]).sort());
    setCode(code);
    console.log(expressions);
  };
  return (
    <Create saveButtonProps={{ ...saveButtonProps }}>
      <Row gutter={{ xs: 8, sm: 16, md: 24 }}>
        <Col span={12}>
          <Form {...formProps} layout="vertical" onFinish={onFinish1}>
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
              name={["project", "id"]}
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
          <TagManager initialData={expressions} />
          <SyntaxHighlighter language="csharp" style={docco} children={code} />
        </Col>
      </Row>
    </Create>
  );
};

import React, { useEffect, useRef, useState } from "react";
import { IResourceComponentsProps } from "@refinedev/core";
import { Edit, useForm, useSelect } from "@refinedev/antd";
import { Col, Form, Input, Row, Select } from "antd";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from "react-syntax-highlighter/dist/esm/styles/hljs";
import TagManager from "../../components/tag-manager";
import { invoke } from "@tauri-apps/api";
const command = "get_expressions";
export const TemplateEdit: React.FC<IResourceComponentsProps> = () => {
  const { formProps, saveButtonProps, queryResult, onFinish } = useForm({});
  const templatesData = queryResult?.data?.data;
  const [code, setCode] = useState("");
  const [expressions, setExpressions] = useState<string[]>([]);
  const { selectProps: projectSelectProps } = useSelect({
    resource: "projects",
    defaultValue: templatesData?.project_id,
    optionLabel: "name",
  });

  const textAreaChange = async (e) => {
    const text = e.target.value;
    setCode(text);
  };

  const preFinish = (values) => {
    onFinish({ ...values, projectId: values.project_id });
  };
  useEffect(() => {
    setCode(queryResult?.data?.data.content);
  }, []);
  useEffect(() => {
    console.log("code", code);
    invoke(command, { template: code }).then((expressions) => {
      setExpressions((expressions as string[]).sort());
    });
  }, [code]);

  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Row gutter={{ xs: 8, sm: 16, md: 24 }}>
        <Col span={12}>
          <Form {...formProps} layout="vertical" onFinish={preFinish}>
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
          <TagManager initialData={expressions} />
          <SyntaxHighlighter
            language="csharp"
            howInlineLineNumbers
            showLineNumbers
            style={docco}
            children={code}
          />
        </Col>
      </Row>
    </Edit>
  );
};

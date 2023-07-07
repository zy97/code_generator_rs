import React, { useEffect, useRef, useState } from "react";
import { IResourceComponentsProps, useCustom } from "@refinedev/core";
import { Edit, useForm, useSelect } from "@refinedev/antd";
import { Col, Form, Input, Row, Select } from "antd";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from "react-syntax-highlighter/dist/esm/styles/hljs";
import TagManager from "../../components/tag-manager";
import { invoke } from "@tauri-apps/api";
const command = "get_expressions";
export const TemplateRender: React.FC<IResourceComponentsProps> = () => {
  const { formProps, saveButtonProps, queryResult, onFinish } = useForm({
    dataProviderName: "tauri",
    action: "edit",
    resource: "templates",
    id: 2,
  });
  //   const sdf = useCustom({ dataProviderName: "ta" });
  const templatesData = queryResult?.data?.data;
  const [code, setCode] = useState("");
  const [expressions, setExpressions] = useState<string[]>([]);
  //   const { selectProps: projectSelectProps } = useSelect({
  //     resource: "projects",
  //     defaultValue: templatesData?.project_id,
  //     optionLabel: "name",
  //   });

  const textAreaChange = async (e) => {
    const text = e.target.value;
    setCode(text);
  };

  const preFinish = (values) => {
    onFinish({ ...values, projectId: values.project_id });
  };
  useEffect(() => {
    setCode(queryResult?.data?.data.content);
  }, [queryResult]);
  useEffect(() => {
    // console.log("code", code);
    // invoke(command, { template: code }).then((expressions) => {
    //   setExpressions((expressions as string[]).sort());
    // });
  }, [code]);

  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Row gutter={{ xs: 8, sm: 16, md: 24 }}>
        <Col span={12}>
          <Form {...formProps} layout="vertical" onFinish={preFinish}>
            <Form.Item
              label="Content"
              name={["content"]}
              rules={[
                {
                  required: true,
                },
              ]}
            ></Form.Item>
          </Form>
          <SyntaxHighlighter
            language="csharp"
            howInlineLineNumbers
            showLineNumbers
            style={docco}
            children={code}
          />
        </Col>
        <Col span={12}>
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

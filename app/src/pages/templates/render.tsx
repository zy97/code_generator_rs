import React, { useEffect, useState } from "react";
import {
  IResourceComponentsProps,
  useNotification,
  useResource,
} from "@refinedev/core";
import { Edit, useForm } from "@refinedev/antd";
import { Col, Form, Input, Row } from "antd";
import { invoke } from "@tauri-apps/api";
import { save } from "@tauri-apps/api/dialog";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from "react-syntax-highlighter/dist/esm/styles/hljs";
const command = "get_expressions";
export const TemplateRender: React.FC<IResourceComponentsProps> = () => {
  const { resources, resource, action, id } = useResource();
  const { open, close } = useNotification();
  const { formProps, saveButtonProps, queryResult, onFinish, form } = useForm({
    dataProviderName: "tauri",
    action: "edit",
    resource: "templates",
    id,
  });
  //   const sdf = useCustom({ dataProviderName: "ta" });
  const templatesData = queryResult?.data?.data;
  const [code, setCode] = useState("");
  const [renderedCode, setRenderedCode] = useState("");
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

  const preFinish = async (values) => {
    const filePath = await save({
      filters: [
        {
          name: "all files",
          extensions: ["*"],
        },
      ],
    });
    if (filePath) {
      console.log("filePath", filePath);
      await invoke("process_to_file", {
        id: queryResult?.data?.data.id,
        expressions: form.getFieldsValue(),
        file: filePath,
      });
      open?.({
        type: "success",
        message: "file saved successfully",
      });

      // onFinish({ ...values, projectId: values.project_id });
    }
  };
  useEffect(() => {
    setCode(queryResult?.data?.data.content);
    setRenderedCode(queryResult?.data?.data.content);
    setExpressions(queryResult?.data?.data.expressions);
    console.log("useResource", resources, resource, action, id);
  }, []);
  const handleInputChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    let expressions: { [key: string]: any } = form.getFieldsValue();
    //update every expression
    for (let expression in expressions) {
      if (!expressions[expression] || expressions[expression].trim() === "") {
        expressions[expression] = `{{${expression}}}`;
      }
    }

    let res = await invoke("process", {
      id: queryResult?.data?.data.id,
      expressions,
    });
    setRenderedCode(res);
  };
  return (
    <Edit saveButtonProps={saveButtonProps} title="Render">
      <Row gutter={{ xs: 8, sm: 16, md: 24 }}>
        <Col span={12}>
          <Form
            {...formProps}
            size="small"
            onFinish={preFinish}
            onChange={handleInputChange}
          >
            {expressions?.map((expression) => {
              return (
                <Form.Item
                  label={expression}
                  name={expression}
                  rules={[
                    {
                      required: true,
                    },
                  ]}
                >
                  <Input />
                </Form.Item>
              );
            })}
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
            children={renderedCode}
          />
        </Col>
      </Row>
    </Edit>
  );
};

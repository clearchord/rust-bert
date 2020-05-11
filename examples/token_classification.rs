// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rust_bert::pipelines::token_classification::{TokenClassificationModel, TokenClassificationConfig, LabelAggregationOption};
use rust_bert::resources::{Resource, RemoteResource};
use rust_bert::bert::{BertModelResources, BertVocabResources, BertConfigResources};
use rust_bert::pipelines::common::ModelType;

fn main() -> failure::Fallible<()> {

//    Load a configuration
    let config = TokenClassificationConfig::new(ModelType::Bert,
                                                Resource::Remote(RemoteResource::from_pretrained(BertModelResources::BERT_NER)),
                                                Resource::Remote(RemoteResource::from_pretrained(BertConfigResources::BERT_NER)),
                                                Resource::Remote(RemoteResource::from_pretrained(BertVocabResources::BERT_NER)),
                                                None, //merges resource only relevant with ModelType::Roberta
                                                false, //lowercase
                                                LabelAggregationOption::Mode,
    );

//    Create the model
    let token_classification_model = TokenClassificationModel::new(config)?;
    let input = [
        "My name is Amélie. I live in Москва.",
        "Chongqing is a city in China."
    ];
    let token_outputs = token_classification_model.predict(&input, true, false); //ignore_first_label = true (only returns the NER parts, ignoring first label O)

    for token in token_outputs {
        println!("{:?}", token);
    }

    Ok(())
}
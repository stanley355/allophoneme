<div align="center">
  <img src="https://user-images.githubusercontent.com/53996155/219663285-097e7a15-790d-49f6-9d7f-16854be477da.png" alt="allophoneme">
</div>

# Allophoneme

Machine Learning Model to find Allophones and Phonemes written in Rust. Any PR are welcome

## Purpose

- This project is intended to help linguist find Allophones and Phonemes easier.
  So far I've seen that linguists are strugglin to find Allophone and Phonemes manually
  hence I tried to make their work easier by creating this repo.

- This repo will provide linguist to encode the IPA (International Phonetic Alphabet) with
  their own customized encoding and from that encoding linguist can find Allophones
  or Phonemes much easier by looking at sorted words which are similar to the target word.
  The similarity rate is based on their IPA encoding, not based on their letters.

## Allophones and Phonemes

Allophones and phonemes are important concepts in the field of linguistics and the study of language. They are used to understand the sounds that are used in spoken language, and how these sounds can vary depending on the context and the speaker.

Phonemes are the smallest units of sound in a language that can distinguish one word from another. For example, in English, the sounds /p/ and /b/ are phonemes, as they can change the meaning of a word (e.g. "pat" vs. "bat"). By identifying the phonemes in a language, linguists can better understand the sound system and structure of the language.

Allophones, on the other hand, are variations of phonemes that are used in different contexts. For example, the /p/ sound in English can be pronounced slightly differently depending on where it appears in a word or sentence. These different pronunciations are called allophones. By identifying allophones, linguists can better understand how language is used in different contexts and how sounds can vary based on the speaker or the environment.

Overall, understanding the concepts of phonemes and allophones is important for analyzing and understanding the sound systems of languages, as well as for developing and teaching language-related technologies such as speech recognition and synthesis.

## Installation

It's pretty simple

- $ `git clone https://github.com/stanley355/allophoneme.git`
- $ `cargo build`
- $ `cargo run`

## !important

1. For the language data to be read, arrange the dataset following the template I've set on `template.xlsx`
2. Put your template on the `same folder` with the repo, `not in` the repo. Example
![image](https://user-images.githubusercontent.com/53996155/219829648-f85c2684-97fa-46b2-947f-25ae713504ee.png)

3. DONT put any `row` or `column` name on the first or top cell, it will affect the data reading.
4. You can put the encoding on `IPA` as you like, but remember it should be a word without any space or symbols
5. If you run the similarity menu, the similarity rate is based on the IPA encoding, not based on their letters.
6. This project is still beta, need to be tested with bigger model

## Screenshots
1. Welcome menu
<div>
  <img src="https://user-images.githubusercontent.com/53996155/219847393-8de02f90-b71d-4af6-bc7f-e0225c79fcb7.png" alt="allophoneme">
</div>

2. Excel Template for Datasets
<div>
  <img src="https://user-images.githubusercontent.com/53996155/219847424-dbfef53f-fd8f-4c6e-aa6a-9d5743052c71.png" alt="allophoneme">
</div>

3. Example flow for similarity finding
<div>
  <img src="https://user-images.githubusercontent.com/53996155/219848074-b0a24afc-cfd8-4d4e-bfd6-df6d27ce8fcf.png" alt="allophoneme">
</div>

4. Example Result
<div>
  <img src="https://user-images.githubusercontent.com/53996155/219848091-c793c123-efbb-493d-8b3d-5655ebb27f55.png" alt="allophoneme">
</div>

### TODO for myself
- [x] Provide menu that loops the words available and user can choose which words they wanna target
- [x] Example result on readme
- [ ] Research on limiting string length result
- [ ] Filter if the same result already exist then clean it
- [ ] Convert this repo into desktop app using Tauri

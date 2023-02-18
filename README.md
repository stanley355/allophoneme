<div align="center">
  <img src="https://user-images.githubusercontent.com/53996155/219663285-097e7a15-790d-49f6-9d7f-16854be477da.png" alt="allophoneme">
</div>

# Allophoneme

Machine Learning Model to find Allophones and Phonemes

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
3. DONT put any `row` or `column` name on the first or top cell, it will affect the data reading.
4. You can put the encoding on `IPA` as you like, but remember it should be a word without any space or symbols
5. If you run the similarity menu, the similarity rate is based on the IPA encoding, not based on their letters.
6. This project is still beta, need to be tested with bigger model

### TODO
- [ ] Provide menu that loops the words available and user can choose which words they wanna target
- [ ] Example result on readme
- [ ] Convert this repo into desktop app using Tauri

### Any PR is welcome!
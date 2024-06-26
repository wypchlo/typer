<template>  
    <header class="text-center py-6 text-gray-50">
        <h1 class="text-4xl font-medium"> {{ set.name }} </h1>
        <p class="font-light text-2xl"> {{ set.description }} </p>
    </header>

    <hr>

    <section class="text-center py-20 flex items-center justify-center gap-4">
        <!-- List of pairs -->
        <div v-for="(pair, index) in pairs" :key="pair" class="flex flex-col justify-around my-2 h-60 w-60 text-center pt-2 bg-gray-100 rounded-xl">

            <div v-if="editingIndex != index" id="NOT EDITING" class="flex flex-col justify-around h-full">
                <div class="flex flex-col">
                    <h1 class="text-2xl font-medium"> {{ pair.word }} </h1>
                    <p class="text-gray-500"> {{ idLangs[pair.wordLanguageId] }} </p>
                    <br>
                    <h1 class="text-2xl font-medium"> {{ pair.translation }} </h1>
                    <p class="text-gray-500"> {{ idLangs[pair.translationLanguageId] }} </p>
                </div>
                
                <div class="flex gap-4 justify-center">
                    <button @click="deletePair(pair._id, index)" class="w-6 h-6"><img src="/src/assets/icons/delete.svg" class="w-full h-full"></button>
                    <button @click="editingIndex = index" class="w-6 h-6"><img src="/src/assets/icons/edit.svg" class="w-full h-full"></button>
                </div>
            </div>

            <EditablePair @confirmed="editPair" v-else :pair="pair" :langs="langs"></EditablePair>
        </div>

        <!-- Add a new pair --> 

        <button v-if="!addingPair" @click="addingPair++" class="flex flex-col items-center justify-around my-2 h-60 w-60 text-center pt-2 bg-gray-100 rounded-xl"><h1 class="text-2xl font-medium"> Add pair </h1></button>

        <EditablePair @confirmed="addPair" v-else :pair="savedPair" :langs="langs"></EditablePair>
    </section>

    <hr>

    <footer class="text-center py-6">
        <h1 class="text-4xl font-medium"><router-link :to="{ name: 'SetPractice', params: { id: set._id } }"> Practice </router-link></h1>
    </footer>
</template>

<script>
    import axios from 'axios';
    import EditablePair from '../components/EditablePair.vue';

    const BASE_API = 'http://192.168.125.199:3000/api';

    export default {
        data() {
            return {
                setId: this.$route.params.id,
                set: {},
                pairs: [],
                langs: [],
                idLangs: {},

                savedPair: {},
                editingIndex: null,
                addingPair: false,
            }
        },
        async mounted() {
            await Promise.all([this.getSet(), this.getPairs(), this.getLanguages()]);
        },
        methods: {
            getPairs: async function() {
                const { data } = await axios.get(`${BASE_API}/pairs/set/${this.setId}`);
                this.pairs = data;
            },
           getSet: async function() {
                const { data } = await axios.get(`${BASE_API}/sets/${this.setId}`);
                this.set = data;
            },
            getLanguages: async function() {
                const { data } = await axios.get(`${BASE_API}/languages`);
                this.langs = data;
                for(let lang of data) this.idLangs[lang['_id']] = lang.language;
            },
            deletePair: async function(id, index) {
                try {
                    await axios.delete(`${BASE_API}/pairs/${id}`);

                    this.pairs.splice(index, 1);
                }
                catch(error) {
                    console.error(`Error while deleting pair: ${error}`);
                }
            },
            addPair: async function(pair) {
                try {
                    pair.setId = this.set._id;
                    await axios.post(`${BASE_API}/pairs/add`, pair);

                    const { wordLanguageId, translationLanguageId } = pair;
                    this.savedPair = { wordLanguageId, translationLanguageId };
                    this.addingPair = false;

                    await this.getPairs();
                }
                catch(error) {
                    console.error(`Error while adding pair to set: ${error}`);
                }
            },
            editPair: async function(updatedPair) {
                try {
                    await axios.put(`${BASE_API}/pairs/${updatedPair._id}`, updatedPair);

                    this.pairs.splice(this.editingIndex, 1, updatedPair);
                    this.editingIndex = null;
                }
                catch(errror) {
                    console.error(`Error while editing pair: ${error}`);
                }
            }
        },
        components: {
            EditablePair
        }
    }
</script>
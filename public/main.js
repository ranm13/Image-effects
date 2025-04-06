async function init(){
    let rustApp = null;
    try {
        rustApp = await import('../pkg')
    }
    catch(e){
        console.error(e)
        return;
    }
    const input = document.getElementById('upload');
    const effectPicker = document.getElementById('effectPicker');
    const fileReader = new FileReader();

    fileReader.onloadend = () => {
        const base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
        const selectedEffect = effectPicker.value;
        let imageDataUrl = rustApp.convert(base64, selectedEffect);
        document.getElementById('new-img').setAttribute('src', imageDataUrl);
    }

    input.addEventListener('change', () => {
        if(input?.files?.length){
            fileReader.readAsDataURL(input.files[0]);
        }
    })

    effectPicker.addEventListener('change', () => {
        if (input?.files?.length && fileReader.readyState === FileReader.DONE) {
            const base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
            const selectedEffect = effectPicker.value;
            let imageDataUrl = rustApp.convert(base64, selectedEffect);
            document.getElementById('new-img').setAttribute('src', imageDataUrl);
        }
    });
}

init();
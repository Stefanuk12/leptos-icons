use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 13a2 2 0 0 0 2-2V7a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0V4a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0v-4a2 2 0 0 1 2-2" ></ path > < / > } } pub const LUCIDE_AUDIO_WAVEFORM : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;
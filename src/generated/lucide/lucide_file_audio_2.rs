use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle r = "1" cy = "17" cx = "3" ></ circle > < path d = "M2 17v-3a4 4 0 0 1 8 0v3" ></ path > < circle r = "1" cy = "17" cx = "9" ></ circle > < / > } } pub const LUCIDE_FILE_AUDIO_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cx = "3" r = "1" cy = "17" ></ circle > < path d = "M2 17v-3a4 4 0 0 1 8 0v3" ></ path > < circle cx = "9" r = "1" cy = "17" ></ circle > < / > } } pub const LUCIDE_FILE_AUDIO_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;
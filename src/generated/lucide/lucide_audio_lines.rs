use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 10v3" ></ path > < path d = "M6 6v11" ></ path > < path d = "M10 3v18" ></ path > < path d = "M14 8v7" ></ path > < path d = "M18 5v13" ></ path > < path d = "M22 10v3" ></ path > < / > } } pub const LUCIDE_AUDIO_LINES : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;
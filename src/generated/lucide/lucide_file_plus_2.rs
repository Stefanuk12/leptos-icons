use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M3 15h6" ></ path > < path d = "M6 12v6" ></ path > < / > } } pub const LUCIDE_FILE_PLUS_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;
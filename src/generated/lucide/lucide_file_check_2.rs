use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "m3 15 2 2 4-4" ></ path > < / > } } pub const LUCIDE_FILE_CHECK_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round")] } ;
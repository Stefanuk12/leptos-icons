use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "m9 15 2 2 4-4" ></ path > < / > } } pub const LUCIDE_FILE_CHECK : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;
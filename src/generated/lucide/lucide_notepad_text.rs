use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M12 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect height = "18" width = "16" rx = "2" y = "4" x = "4" ></ rect > < path d = "M8 10h6" ></ path > < path d = "M8 14h8" ></ path > < path d = "M8 18h5" ></ path > < / > } } pub const LUCIDE_NOTEPAD_TEXT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;
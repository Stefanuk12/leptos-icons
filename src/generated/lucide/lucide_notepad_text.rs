use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M12 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect rx = "2" x = "4" height = "18" width = "16" y = "4" ></ rect > < path d = "M8 10h6" ></ path > < path d = "M8 14h8" ></ path > < path d = "M8 18h5" ></ path > < / > } } pub const LUCIDE_NOTEPAD_TEXT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;
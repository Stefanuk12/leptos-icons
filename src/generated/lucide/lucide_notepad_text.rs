use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M12 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect x = "4" y = "4" height = "18" width = "16" rx = "2" ></ rect > < path d = "M8 10h6" ></ path > < path d = "M8 14h8" ></ path > < path d = "M8 18h5" ></ path > < / > } } pub const LucideNotepadText : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;
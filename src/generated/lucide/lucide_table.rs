use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v18" ></ path > < rect y = "3" rx = "2" height = "18" width = "18" x = "3" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M3 15h18" ></ path > < / > } } pub const LUCIDE_TABLE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" height = "18" width = "18" y = "3" ></ rect > < path d = "M8 12h8" ></ path > < path d = "M12 8v8" ></ path > < / > } } pub const LUCIDE_SQUARE_PLUS : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;
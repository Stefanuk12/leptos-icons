use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" width = "18" x = "3" height = "18" ></ rect > < path d = "M12 8v8" ></ path > < path d = "m8.5 14 7-4" ></ path > < path d = "m8.5 10 7 4" ></ path > < / > } } pub const LUCIDE_SQUARE_ASTERISK : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
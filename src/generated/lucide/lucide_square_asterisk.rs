use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" height = "18" x = "3" y = "3" ></ rect > < path d = "M12 8v8" ></ path > < path d = "m8.5 14 7-4" ></ path > < path d = "m8.5 10 7 4" ></ path > < / > } } pub const LUCIDE_SQUARE_ASTERISK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;
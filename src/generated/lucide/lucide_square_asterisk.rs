use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "2" height = "18" width = "18" ></ rect > < path d = "M12 8v8" ></ path > < path d = "m8.5 14 7-4" ></ path > < path d = "m8.5 10 7 4" ></ path > < / > } } pub const LUCIDE_SQUARE_ASTERISK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "18" rx = "2" width = "18" ></ rect > < path d = "m15 9-6 6" ></ path > < path d = "M9 9h.01" ></ path > < path d = "M15 15h.01" ></ path > < / > } } pub const LUCIDE_SQUARE_PERCENT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;
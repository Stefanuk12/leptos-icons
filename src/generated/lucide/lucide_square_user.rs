use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" rx = "2" width = "18" y = "3" ></ rect > < circle cx = "12" cy = "10" r = "3" ></ circle > < path d = "M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" ></ path > < / > } } pub const LUCIDE_SQUARE_USER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;
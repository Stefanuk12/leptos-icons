use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" y = "3" x = "3" rx = "2" ></ rect > < circle r = "3" cx = "12" cy = "10" ></ circle > < path d = "M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" ></ path > < / > } } pub const LUCIDE_SQUARE_USER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" y = "3" width = "18" rx = "2" ></ rect > < circle r = "3" cy = "10" cx = "12" ></ circle > < path d = "M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" ></ path > < / > } } pub const LUCIDE_SQUARE_USER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" height = "18" rx = "2" y = "3" ></ rect > < circle cx = "12" r = "3" cy = "10" ></ circle > < path d = "M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" ></ path > < / > } } pub const LUCIDE_SQUARE_USER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24")] } ;
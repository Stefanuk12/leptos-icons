use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 2v2" ></ path > < path d = "M7 22v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" ></ path > < path d = "M8 2v2" ></ path > < circle cy = "11" r = "3" cx = "12" ></ circle > < rect x = "3" rx = "2" width = "18" height = "18" y = "4" ></ rect > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;
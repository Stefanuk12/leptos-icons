use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 2v2" ></ path > < path d = "M7 22v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" ></ path > < path d = "M8 2v2" ></ path > < circle cx = "12" cy = "11" r = "3" ></ circle > < rect rx = "2" y = "4" height = "18" x = "3" width = "18" ></ rect > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 2v2" ></ path > < path d = "M17.915 22a6 6 0 0 0-12 0" ></ path > < path d = "M8 2v2" ></ path > < circle cy = "12" r = "4" cx = "12" ></ circle > < rect width = "18" y = "4" rx = "2" height = "18" x = "3" ></ rect > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;
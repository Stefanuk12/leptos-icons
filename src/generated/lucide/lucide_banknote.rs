use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" y = "6" rx = "2" height = "12" x = "2" ></ rect > < circle r = "2" cx = "12" cy = "12" ></ circle > < path d = "M6 12h.01M18 12h.01" ></ path > < / > } } pub const LUCIDE_BANKNOTE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "6" height = "12" width = "20" x = "2" ></ rect > < circle cx = "12" r = "2" cy = "12" ></ circle > < path d = "M6 12h.01M18 12h.01" ></ path > < / > } } pub const LUCIDE_BANKNOTE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
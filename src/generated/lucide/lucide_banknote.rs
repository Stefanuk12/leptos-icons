use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "6" rx = "2" width = "20" height = "12" x = "2" ></ rect > < circle r = "2" cx = "12" cy = "12" ></ circle > < path d = "M6 12h.01M18 12h.01" ></ path > < / > } } pub const LUCIDE_BANKNOTE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;
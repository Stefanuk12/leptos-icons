use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h3" ></ path > < path d = "M17 6h.01" ></ path > < rect y = "2" x = "3" width = "18" rx = "2" height = "20" ></ rect > < circle r = "5" cx = "12" cy = "13" ></ circle > < path d = "M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5" ></ path > < / > } } pub const LUCIDE_WASHING_MACHINE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;
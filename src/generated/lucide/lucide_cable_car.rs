use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 3h.01" ></ path > < path d = "M14 2h.01" ></ path > < path d = "m2 9 20-5" ></ path > < path d = "M12 12V6.5" ></ path > < rect width = "16" x = "4" height = "10" y = "12" rx = "3" ></ rect > < path d = "M9 12v5" ></ path > < path d = "M15 12v5" ></ path > < path d = "M4 17h16" ></ path > < / > } } pub const LUCIDE_CABLE_CAR : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;
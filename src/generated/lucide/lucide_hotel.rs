use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 22v-6.57" ></ path > < path d = "M12 11h.01" ></ path > < path d = "M12 7h.01" ></ path > < path d = "M14 15.43V22" ></ path > < path d = "M15 16a5 5 0 0 0-6 0" ></ path > < path d = "M16 11h.01" ></ path > < path d = "M16 7h.01" ></ path > < path d = "M8 11h.01" ></ path > < path d = "M8 7h.01" ></ path > < rect x = "4" rx = "2" width = "16" y = "2" height = "20" ></ rect > < / > } } pub const LUCIDE_HOTEL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24")] } ;
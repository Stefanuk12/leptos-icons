use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 22v-6.57" ></ path > < path d = "M12 11h.01" ></ path > < path d = "M12 7h.01" ></ path > < path d = "M14 15.43V22" ></ path > < path d = "M15 16a5 5 0 0 0-6 0" ></ path > < path d = "M16 11h.01" ></ path > < path d = "M16 7h.01" ></ path > < path d = "M8 11h.01" ></ path > < path d = "M8 7h.01" ></ path > < rect height = "20" rx = "2" x = "4" y = "2" width = "16" ></ rect > < / > } } pub const LUCIDE_HOTEL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;
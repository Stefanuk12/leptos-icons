use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" height = "20" y = "2" ry = "2" width = "16" rx = "2" ></ rect > < path d = "M9 22v-4h6v4" ></ path > < path d = "M8 6h.01" ></ path > < path d = "M16 6h.01" ></ path > < path d = "M12 6h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M16 10h.01" ></ path > < path d = "M16 14h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M8 14h.01" ></ path > < / > } } pub const LUCIDE_BUILDING : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;
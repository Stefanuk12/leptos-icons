use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "20" x = "4" y = "2" rx = "2" ry = "2" ></ rect > < path d = "M9 22v-4h6v4" ></ path > < path d = "M8 6h.01" ></ path > < path d = "M16 6h.01" ></ path > < path d = "M12 6h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M16 10h.01" ></ path > < path d = "M16 14h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M8 14h.01" ></ path > < / > } } pub const LUCIDE_BUILDING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
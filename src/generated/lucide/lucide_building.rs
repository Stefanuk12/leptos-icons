use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 22v-4h6v4" ></ path > < path d = "M8 6h.01" ></ path > < path d = "M16 6h.01" ></ path > < path d = "M12 6h.01" ></ path > < path d = "M12 10h.01" ></ path > < path d = "M12 14h.01" ></ path > < path d = "M16 10h.01" ></ path > < path d = "M16 14h.01" ></ path > < path d = "M8 10h.01" ></ path > < path d = "M8 14h.01" ></ path > < / > } } pub const LucideBuilding : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;
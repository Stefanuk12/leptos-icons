use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z" ></ path > < path d = "M8 13v9" ></ path > < path d = "M16 22v-9" ></ path > < path d = "m9 6 1 7" ></ path > < path d = "m15 6-1 7" ></ path > < path d = "M12 6V2" ></ path > < path d = "M13 2h-2" ></ path > < / > } } pub const LUCIDE_TOWER_CONTROL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;
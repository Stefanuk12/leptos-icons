use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "M16 14v2" ></ path > < path d = "M8 14v2" ></ path > < path d = "M16 20h.01" ></ path > < path d = "M8 20h.01" ></ path > < path d = "M12 16v2" ></ path > < path d = "M12 22h.01" ></ path > < / > } } pub const LucideCloudHail : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;
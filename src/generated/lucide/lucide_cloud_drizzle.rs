use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "M8 19v1" ></ path > < path d = "M8 14v1" ></ path > < path d = "M16 19v1" ></ path > < path d = "M16 14v1" ></ path > < path d = "M12 21v1" ></ path > < path d = "M12 16v1" ></ path > < / > } } pub const LUCIDE_CLOUD_DRIZZLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
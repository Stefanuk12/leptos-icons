use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cx = "12" rx = "9" ry = "3" cy = "5" ></ ellipse > < path d = "M3 5V19A9 3 0 0 0 21 19V5" ></ path > < path d = "M3 12A9 3 0 0 0 21 12" ></ path > < / > } } pub const LUCIDE_DATABASE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;
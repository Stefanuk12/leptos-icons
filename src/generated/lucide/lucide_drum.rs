use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 2 8 8" ></ path > < path d = "m22 2-8 8" ></ path > < ellipse cx = "12" cy = "9" ry = "5" rx = "10" ></ ellipse > < path d = "M7 13.4v7.9" ></ path > < path d = "M12 14v8" ></ path > < path d = "M17 13.4v7.9" ></ path > < path d = "M2 9v8a10 5 0 0 0 20 0V9" ></ path > < / > } } pub const LUCIDE_DRUM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;
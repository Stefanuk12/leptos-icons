use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cy = "5" ry = "3" rx = "9" cx = "12" ></ ellipse > < path d = "M3 5v14a9 3 0 0 0 18 0V5" ></ path > < / > } } pub const LUCIDE_CYLINDER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;
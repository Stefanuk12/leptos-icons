use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98" ></ path > < ellipse cy = "19" cx = "12" rx = "9" ry = "3" ></ ellipse > < / > } } pub const LUCIDE_CONE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24")] } ;
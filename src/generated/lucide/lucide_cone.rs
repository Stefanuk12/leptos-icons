use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98" ></ path > < ellipse cx = "12" ry = "3" rx = "9" cy = "19" ></ ellipse > < / > } } pub const LUCIDE_CONE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;
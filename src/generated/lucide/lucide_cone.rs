use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98" ></ path > < ellipse cx = "12" ry = "3" rx = "9" cy = "19" ></ ellipse > < / > } } pub const LUCIDE_CONE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2")] } ;
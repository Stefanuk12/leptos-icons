use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 13v8" ></ path > < path d = "M12 3v3" ></ path > < path d = "M18 6a2 2 0 0 1 1.414.586l2.293 2.207a1 1 0 0 1 0 1.414l-2.27 2.184a2 2 0 0 1-1.742.586L6 13a2 2 0 0 1-1.414-.586l-2.293-2.207a1 1 0 0 1 0-1.414l2.293-2.207A2 2 0 0 1 6 6z" ></ path > < / > } } pub const LUCIDE_SIGNPOST : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3" ></ path > < path d = "m16 16 5 5" ></ path > < path d = "m16 21 5-5" ></ path > < / > } } pub const LUCIDE_GRID_2_X_2_X : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M6 18 18 6M6 6l12 12" ></ path > < / > } } pub const HEROICONS_OUTLINE_X_MARK : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;
use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M15.042 21.672 13.684 16.6m0 0-2.51 2.225.569-9.47 5.227 7.917-3.286-.672ZM12 2.25V4.5m5.834.166-1.591 1.591M20.25 10.5H18M7.757 14.743l-1.59 1.59M6 10.5H3.75m4.007-4.243-1.59-1.59" ></ path > < / > } } pub const HEROICONS_OUTLINE_CURSOR_ARROW_RAYS : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;